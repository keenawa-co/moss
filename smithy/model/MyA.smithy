$version: "2"
namespace moss.example

service MinimalService {
    version: "2024-10-15",
    operations: []
}

structure MyA {
    myB: MyB
}

structure MyB {
    some: String
}