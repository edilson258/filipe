import io
import random

io.puts("Welcom to the Game")

define main(): void {
  let expected = random.randint(1, 10)

  io.puts("I have a number from 1 to 10")
  io.puts("Guess it. U have 5 chances")

  let guess: int

  for count in range(0, 5) {
    guess = io.gets("Your Guess: ").as_int()

    if guess > expected {
      io.puts(guess, " is too high")
    }

    if guess < expected {
      io.puts(guess, " is too low")
    }

    if guess == expected {
      io.puts("You Got it. Congrats!")
      return null
    }
  }

  io.puts("Your lost, try again!")
}

main()
