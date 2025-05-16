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
