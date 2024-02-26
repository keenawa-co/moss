name = getenv("TERM")


job "define your job description here" {
    watch = ["*"]

    include "github.com/4rchr4y/gst" "latest" {
        comment = "Step decription here (optional)"
        use = {
            "goray.*": "ERROR"
        }
        rules = [
            "$CI_PIPELINE_SOURCE" == "push" 
        ]
    }

    include "github.com/4rchr4y/gst" "v1.0.1" {
        use = {
            "goray.SE0001": "ERROR"
            "goray.SE0002": "WARN"
        }
        // rules = [
        //    "${var.name == "Example" ? true : false}"
        // ]
    }
}

