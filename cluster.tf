provider "azurerm" {
  features {}
}

resource "azurerm_resource_group" "devo-rg" {
  name     = "devo-cluster-rg"
  location = "UK South"
}

resource "random_id" "cr-id" {
  byte_length = 5
}

resource "azurerm_container_registry" "cluster-cr" {
  name                = "clustercr${random_id.cr-id.hex}"
  location            = azurerm_resource_group.devo-rg.location
  resource_group_name = azurerm_resource_group.devo-rg.name
  sku                 = "Standard"
}

resource "azurerm_kubernetes_cluster" "devo-cluster" {
  name                = "devo-cluster"
  location            = azurerm_resource_group.devo-rg.location
  resource_group_name = azurerm_resource_group.devo-rg.name
  dns_prefix          = "devo-cluster"

  default_node_pool {
    name       = "devonp"
    node_count = 1
    vm_size    = "standard_b2s"
  }

  identity {
    type = "SystemAssigned"
  }

  role_based_access_control_enabled = true
}

resource "azurerm_role_assignment" "devo-cluster-acr-role" {
  scope                = azurerm_container_registry.cluster-cr.id
  role_definition_name = "AcrPull"
  principal_id         = azurerm_kubernetes_cluster.devo-cluster.kubelet_identity[0].object_id
}

provider "kubectl" {
  load_config_file       = false
  host                   = azurerm_kubernetes_cluster.devo-cluster.kube_config[0].host
  token                  = azurerm_kubernetes_cluster.devo-cluster.kube_config[0].password
  cluster_ca_certificate = base64decode(azurerm_kubernetes_cluster.devo-cluster.kube_config[0].cluster_ca_certificate)
}

data "kubectl_path_documents" "docs" {
  pattern = "*.yaml"
}

resource "kubectl_manifest" "manifests" {
  for_each  = toset(data.kubectl_path_documents.docs.documents)
  yaml_body = each.value
}
