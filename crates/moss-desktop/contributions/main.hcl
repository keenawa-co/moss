pragma required_version ">= 1.0" {}

locals {
    max_subnet_length = 10
}

configuration "moss.core.window" {
    title = "Window"
    order = 5

    parameter "window.defaultWidth" {
        type = number
        minimum = 800
        maximum = 3840
        default = 800
        order = 1
        scope = "APPLICATION"
        description = "The width of the application window in pixels."
    }

    parameter "window.defaultHeight" {
        type = number
        minimum = 600
        maximum = 2160
        default = 600
        order = 2
        scope = "APPLICATION"
        description = "The height of the application window in pixels."
    }

    parameter "editor.fontSize" {
        type = number
        minimum = 10
        maximum = 20
        default = 14
        order = 1
        scope = "WINDOW"
        description = "The width of the application window in pixels."
    }

    override "editor.fontSize" {
        value = 16
        context = [
            "typescript",
            "javascript"
        ]
    }
}

configuration {
    override "editor.fontSize" {
        value = 16
        context = [
            "typescript",
            "javascript"
        ]
    }
}
