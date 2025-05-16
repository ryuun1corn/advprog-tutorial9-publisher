> a. How much data will your publisher program send to the message broker in one run?

Program publisher akan mengirimkan 5 buah pesan ke message broker dalam satu kali dijalankan. Masing-masing pesan bertipe UserCreatedEventMessage, yang berisi dua field: user_id dan user_name. Karena setiap field bertipe String, besar data aktual yang dikirim akan tergantung pada panjang karakter dalam string tersebut. Namun secara struktur, total data mencakup 5 entri ID dan nama pengguna, yang diserialisasi menggunakan format Borsh, sebuah format biner efisien. Jadi, meskipun ukuran pasti dalam byte tergantung pada panjang string, intinya adalah bahwa ada 5 unit pesan dikirim ke topik user_created.

> b. The URL of: amqp://guest:guest@localhost:5672 is the same as in the subscriber program, what does it mean?

Fakta bahwa URL koneksi amqp://guest:guest@localhost:5672 sama dengan yang digunakan di program subscriber berarti bahwa publisher dan subscriber terhubung ke message broker yang sama, yaitu RabbitMQ yang berjalan di mesin lokal (localhost) melalui port 5672. Kredensial guest:guest menunjukkan bahwa keduanya menggunakan akun default untuk autentikasi. Dengan ini, pesan yang dikirim oleh publisher ke routing key user_created dapat langsung diterima oleh subscriber yang sedang mendengarkan key tersebut di broker yang sama.
