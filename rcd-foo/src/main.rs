use tracing::info;
use tracing_subscriber::{self, util::SubscriberInitExt};

fn main() {

    let subscriber = tracing_subscriber::fmt().compact().with_file(true).with_line_number(true).with_target(true).finish();
    subscriber.init();

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");

    let number_shaved = yak_shave::shave_all(number_of_yaks);
    info!(
        all_yaks_shaved = number_shaved == number_of_yaks,
        "yak shaving completed."
    );
}

mod yak_shave{
    use tracing::info;

    pub fn shave_all(number_of_yaks: u32) -> u32 {
        info!("this is a message");
        number_of_yaks
    }
}