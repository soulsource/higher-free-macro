//! this module interprets the domain specific language as a text adventure.

use crate::dsl::FreeSausageRoll;

pub fn run<'a, 's: 'a>(mut game: FreeSausageRoll<'a, 's, ()>) -> std::io::Result<()> {
    //this function doesn't know who it is, or why it is here. It only knows it must deal.
    //Deal with the few commands in the eDSL and nothing more.

    //This would be easier to write recursively. However, in an actual project this might run for quite some time.
    //Since we operate on the stack, let's rather be safe than sorry, and use a loop instead of recursion therefore.
    while let FreeSausageRoll::Free(command) = game {
        game = match *command {
            crate::dsl::SausageRoll::SayDialogueLine {
                speaker,
                text,
                mood,
                next,
            } => {
                println!(
                    "{} says: \"{text}\" with {} on their face.",
                    speaker.text_description(),
                    mood.text_description()
                );
                next
            }
            crate::dsl::SausageRoll::GivePlayerOptions { options, next } => {
                println!("Your options are:");
                for (id, option) in options.iter().enumerate().map(|(i, o)| (i + 1, o)) {
                    println!("{id}: {option}");
                }

                let mut input = String::new();
                let mut chosen;
                while {
                    input.clear();
                    std::io::stdin().read_line(&mut input)?;
                    chosen = input
                        .trim()
                        .parse()
                        .ok()
                        .filter(|o: &usize| *o > 0 && *o <= options.len());
                    chosen.is_none()
                } {
                    println!("Invalid choice. Please select one of the options given above.");
                }
                println!();
                next(chosen.unwrap() - 1)
            }
            crate::dsl::SausageRoll::PresentLocation { location, next } => {
                println!("{}", location.get_text_description());
                next
            }
            crate::dsl::SausageRoll::Exposition { text, next } => {
                println!("{text}");
                next
            }
        };
    }
    std::io::Result::Ok(())
}
