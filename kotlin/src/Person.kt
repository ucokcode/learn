class Person(val firstName: String, val lastName: String) {
    init {
        println("init 1")
    }
    constructor(): this("Harapan", "Pardamean") {
        println("secondary constructor")
    }
    init {
        println("init 2")
    }
}