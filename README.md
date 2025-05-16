> a. How much data will your publisher program send to the message broker in one run?

Program publisher akan mengirimkan 5 buah pesan ke message broker dalam satu kali dijalankan. Masing-masing pesan bertipe UserCreatedEventMessage, yang berisi dua field: user_id dan user_name. Karena setiap field bertipe String, besar data aktual yang dikirim akan tergantung pada panjang karakter dalam string tersebut. Namun secara struktur, total data mencakup 5 entri ID dan nama pengguna, yang diserialisasi menggunakan format Borsh, sebuah format biner efisien. Jadi, meskipun ukuran pasti dalam byte tergantung pada panjang string, intinya adalah bahwa ada 5 unit pesan dikirim ke topik user_created.

> b. The URL of: amqp://guest:guest@localhost:5672 is the same as in the subscriber program, what does it mean?

Fakta bahwa URL koneksi amqp://guest:guest@localhost:5672 sama dengan yang digunakan di program subscriber berarti bahwa publisher dan subscriber terhubung ke message broker yang sama, yaitu RabbitMQ yang berjalan di mesin lokal (localhost) melalui port 5672. Kredensial guest:guest menunjukkan bahwa keduanya menggunakan akun default untuk autentikasi. Dengan ini, pesan yang dikirim oleh publisher ke routing key user_created dapat langsung diterima oleh subscriber yang sedang mendengarkan key tersebut di broker yang sama.

## RabbitMQ Screenshot

### Web Interface

![rabbitmq screenshot](./images/rabbitmq_screenshot.png)

### Message Passing Example

When running the publisher (`cargo run`), it sends 5 `UserCreatedEventMessage` events to the message broker.  
The subscriber (running in a separate terminal) listens to the `user_created` queue and processes each event.

![message passing screenshot](./images/message_passing_screenshot.png)

### Spikes on the Interface

When the publisher is run, it sends 5 `UserCreatedEventMessage` events at once to the `user_created` queue.  
If you observe the message broker dashboard (e.g., RabbitMQ UI), you'll notice **spikes** on the message rate chart.

These spikes represent a burst of traffic caused by the publisher sending multiple events rapidly.

![rabbitmq spikes screenshot](./images/rabbitmq_spikes_screenshot.png)

### Multiple Items in Queue with Slow Subscriber

When the publisher keeps sending events continuously, those events are placed into the message queue on the broker. The producer can send requests rapidly, while the consumer processes each message one by one in order.

Because of this, the queue can build up if the consumer cannot keep up with the speed of the producer. As a result, the total number of messages in the queue may grow to dozens or more, depending on how fast events are sent and how fast they are processed.

![slow subscriber screenshot](./images/slow_subscriber_queue_screenshot.png)

### Multiple Subscribers Impact

When multiple subscribers are running and listening to the same queue or routing key, the message processing load is distributed among them. This means each subscriber processes only a portion of the total incoming messages, allowing the queue to be emptied faster. As a result, the spike in the message queue seen on the dashboard reduces more quickly than when only a single subscriber is consuming events.

![multiple subscriber screenshot](./images/multiple_subscriber_screenshot.png)
![reduced message spike screenshot](./images/reduced_message_spike_screenshot.png)

## Bonus: Experiment on the Cloud

### Setup

1. Create a RabbitMQ Cloud Instance.

![cloud amqp screenshot](./images/cloudamqp_screenshot.png)
![cloud rabbitmq screenshot](./images/cloud_rabbitmq_screenshot.png)

2. Configure VPS firewall to accept traffic from port 5672.

![vps firewall screenshot](./images/vps_firewall_screenshot.png)

3. Deploy subscriber app on the VPS.

![coolify deployment screenshot](./images/coolify_deployment_screenshot.png)

4. Run publisher app from local machine.

### CLOUD: Message Passing Example

When running the publisher (`cargo run`), it sends 30 `UserCreatedEventMessage` events to the message broker on the cloud.  
The subscriber (running in a VPS) listens to the `user_created` queue and processes each event.

![cloud message passing 2](./images/cloud_message_passing_2.png)
![cloud message passing 1](./images/cloud_message_passing_1.png)

### CLOUD: Spikes on the Interface

When the publisher is run, it sends 30 `UserCreatedEventMessage` events at once to the `user_created` queue.  
If you observe the message broker dashboard (on the LavinMQ interface), you'll notice **spikes** on the message rate chart.

These spikes represent a burst of traffic caused by the publisher sending multiple events rapidly.

![cloud rabbitmq spikes](./images/cloud_rabbitmq_spikes.png)

### CLOUD: Multiple Items in Queue with Slow Subscriber

The Rust application on the VPS already has a delay when processing each request, which simulates a slow subscriber.

The effect is further intensified when running multiple publishers on different terminals.

![cloud multiple publishers screenshot](./images/cloud_multiple_publishers_screenshot.png)
![cloud rabbitmq multiple items impact screenshot](./images/cloud_rabbitmq_multiple_items_impact_screenshot.png)

### CLOUD: Multiple Subscribers Impact

When multiple subscribers are running and listening to the same queue or routing key, the message processing load is distributed among them. This means each subscriber processes only a portion of the total incoming messages, allowing the queue to be emptied faster. As a result, the spike in the message queue seen on the dashboard reduces more quickly than when only a single subscriber is consuming events.

Run additional subscribers on the local machine to help the subscriber on the VPS.

![cloud multiple subscriber screenshot](./images/cloud_multiple_subscriber_screenshot.png)
![cloud reduced message spike screenshot](./images/cloud_reduced_message_spike_screenshot.png)
