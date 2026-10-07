use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use openaction::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    sync::Mutex,
    task::JoinHandle,
    time::sleep,
};

const ACTION_UUID: &str = "de.spliter90.w20.roll";
const ROLL_DURATION: Duration = Duration::from_secs(3);

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct W20Settings {}

#[derive(Default)]
struct SharedState {
    rolls: Mutex<HashMap<String, JoinHandle<()>>>,
}

impl SharedState {
    async fn start_roll(&self, instance_id: &str) {
        if let Some(previous) = self.rolls.lock().await.remove(instance_id) {
            previous.abort();
        }

        let task_instance_id = instance_id.to_owned();
        let task = tokio::spawn(async move {
            let started = Instant::now();

            while started.elapsed() < ROLL_DURATION {
                let value = random_w20();
                render_number(&task_instance_id, value, true).await;

                let elapsed = started.elapsed();
                let delay = if elapsed < Duration::from_millis(2200) {
                    Duration::from_millis(85)
                } else if elapsed < Duration::from_millis(2700) {
                    Duration::from_millis(135)
                } else {
                    Duration::from_millis(210)
                };

                let remaining = ROLL_DURATION.saturating_sub(started.elapsed());
                if remaining.is_zero() {
                    break;
                }
                sleep(if remaining < delay { remaining } else { delay }).await;
            }

            let result = random_w20();
            render_number(&task_instance_id, result, false).await;
        });

        self.rolls
            .lock()
            .await
            .insert(instance_id.to_owned(), task);
    }

    async fn stop_roll(&self, instance_id: &str) {
        if let Some(task) = self.rolls.lock().await.remove(instance_id) {
            task.abort();
        }
    }
}

struct W20Action {
    shared: Arc<SharedState>,
}

#[async_trait]
impl Action for W20Action {
    const UUID: ActionUuid = ACTION_UUID;
    type Settings = W20Settings;

    async fn will_appear(
        &self,
        instance: &Instance,
        _settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        render_idle(instance).await
    }

    async fn key_down(
        &self,
        instance: &Instance,
        _settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        self.shared.start_roll(&instance.instance_id).await;
        Ok(())
    }

    async fn will_disappear(
        &self,
        instance: &Instance,
        _settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        self.shared.stop_roll(&instance.instance_id).await;
        Ok(())
    }
}

fn random_w20() -> u8 {
    rand::thread_rng().gen_range(1..=20)
}

async fn render_idle(instance: &Instance) -> OpenActionResult<()> {
    instance
        .set_image(Some(dice_image_data_url(None, false)), None)
        .await?;
    instance.set_title(Some(""), None).await?;
    Ok(())
}

async fn render_number(instance_id: &str, value: u8, rolling: bool) {
    let Some(instance) = get_instance(instance_id.to_owned()).await else {
        return;
    };

    if let Err(error) = instance
        .set_image(Some(dice_image_data_url(Some(value), rolling)), None)
        .await
    {
        log::warn!("Could not update W20 image: {error}");
    }

    if let Err(error) = instance.set_title(Some(""), None).await {
        log::warn!("Could not clear W20 title: {error}");
    }
}

fn dice_image_data_url(value: Option<u8>, rolling: bool) -> String {
    let (background, accent, label) = match (value, rolling) {
        (_, true) => ("#24163A", "#8E6BE8", "WÜRFELT…"),
        (Some(20), false) => ("#12351E", "#45B96B", "ERGEBNIS"),
        (Some(1), false) => ("#3C1518", "#E0525C", "ERGEBNIS"),
        (Some(_), false) => ("#17243E", "#597DD8", "ERGEBNIS"),
        (None, false) => ("#24163A", "#8E6BE8", "DRÜCKEN"),
    };

    let number = value
        .map(|number| number.to_string())
        .unwrap_or_else(|| "20".to_owned());

    let number_size = if number.len() == 1 { 64 } else { 56 };

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="144" height="144" viewBox="0 0 144 144">
<rect width="144" height="144" rx="20" fill="{background}"/>
<polygon points="72,13 118,40 128,87 98,126 46,126 16,87 26,40" fill="{accent}" fill-opacity=".88" stroke="#FFFFFF" stroke-opacity=".88" stroke-width="3"/>
<path d="M72 13 72 126M26 40 118 40M16 87 128 87M26 40 72 87 118 40M16 87 46 126 72 87 98 126 128 87" fill="none" stroke="#FFFFFF" stroke-opacity=".24" stroke-width="2"/>
<rect x="39" y="47" width="66" height="58" rx="13" fill="#0E0B16" fill-opacity=".76"/>
<text x="72" y="89" text-anchor="middle" font-family="Arial,sans-serif" font-size="{number_size}" font-weight="800" fill="#FFFFFF">{number}</text>
<text x="72" y="118" text-anchor="middle" font-family="Arial,sans-serif" font-size="12" font-weight="700" letter-spacing="1.2" fill="#FFFFFF">{label}</text>
</svg>"#
    );

    format!(
        "data:image/svg+xml;base64,{}",
        BASE64.encode(svg.as_bytes())
    )
}

#[tokio::main]
async fn main() -> OpenActionResult<()> {
    use simplelog::*;

    if let Err(error) = TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Never,
    ) {
        eprintln!("Logger initialization failed: {error}");
    }

    let shared = Arc::new(SharedState::default());
    register_action(W20Action { shared }).await;
    run(std::env::args().collect()).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_value_is_in_w20_range() {
        for _ in 0..500 {
            assert!((1..=20).contains(&random_w20()));
        }
    }

    #[test]
    fn final_result_is_rendered_as_svg_data_url() {
        let image = dice_image_data_url(Some(20), false);
        assert!(image.starts_with("data:image/svg+xml;base64,"));
    }
}
