pragma required_version ">= 1.0" {}

locals {
    max_subnet_length = 10
}

extends "configuration" {
    order = 5

    parameter "window.defaultWidth" {
        type = number
        minimum = 800
        maximum = 3840
        default = 800
        description = "The width of the application window in pixels."
    }

    parameter "window.defaultHeight" {
        type = number
        minimum = 600
        maximum = 2160
        default = 600
        description = "The height of the application window in pixels."
    }
}
