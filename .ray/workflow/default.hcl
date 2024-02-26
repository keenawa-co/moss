default {
    bundle "github.com/4rchr4y/gst" "latest" {
        use = {
            "goray.*": "ERROR"
        }
        rules = [
            "$CI_PIPELINE_SOURCE" == "push" ? true : false
        ]
    }
}


