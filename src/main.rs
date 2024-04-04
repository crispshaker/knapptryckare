#[tokio::main]
async fn main() {
    let mac_addr: String = std::env::args().nth(1).expect("No mac address provided.");
    switchbot_bot(mac_addr.trim(), vec![0x57, 0x01, 0x00])
        .await
        .expect("Failed to move switchbot.");
}

async fn switchbot_bot(mac_addr: &str, payload: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let adapter: bluest::Adapter = bluest::Adapter::default()
        .await
        .ok_or("Bluetooth adapter not found")?;
    adapter.wait_available().await?;

    while let Some(discovered_device) = futures_lite::StreamExt::next(&mut adapter.scan(&[]).await?).await {
        if format!("{:?}", discovered_device.device.id()).contains(mac_addr) {
            let device: bluest::Device = discovered_device.device;
            adapter.connect_device(&device).await?;
            for service in device.services().await? {
                for characteristic in service.characteristics().await? {
                    characteristic.write_without_response(&payload).await?;
                }
            }
            adapter.disconnect_device(&device).await?;
            break;
        }
    }
    Ok(())
}
