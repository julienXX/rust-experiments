// Parallel Hello world

fn main() {
    for num in range(0, 100) {
        do spawn {
            println("Hello " + num.to_str());
        }
    }
}
