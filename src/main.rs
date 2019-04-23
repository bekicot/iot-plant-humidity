extern crate rumqtt;

use rumqtt::{MqttClient, MqttOptions, QoS, ReconnectOptions};

use std::{thread, time::Duration};

fn main() {
    let broker = "localhost";
    let port = 1883;

    let reconnection_options = ReconnectOptions::Always(10);
    let mqtt_options = MqttOptions::new("status_tanaman", broker, port)
                                    .set_keep_alive(10)
                                    .set_reconnect_opts(reconnection_options)
                                    .set_clean_session(false);

    let (mut mqtt_client, _) = MqttClient::start(mqtt_options).unwrap();

    let child = thread::spawn(move || {
        for i in 0..100 {
            let payload = format!("{}", i);
            thread::sleep(Duration::from_millis(100));
            mqtt_client.publish("status_tanaman/suhu_lingkungan", QoS::AtLeastOnce, false, payload).unwrap();
        }
    });
    child.join().unwrap();  
}

