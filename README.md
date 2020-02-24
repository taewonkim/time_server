# time_server

sudo apt-get install -y musl-tools

---

rustup target add x86_64-unknown-linux-musl

(or)

rustup target add x86_64-unknown-linux-musl --toolchain=nightly

---

cargo build --target x86_64-unknown-linux-musl

(or)

cargo build --release --target x86_64-unknown-linux-musl

---

sudo docker build -t time_server:latest .

---

sudo docker run --rm -i -t -p 5000:5000 time_server:latest

(or)

sudo docker run --rm -d -p 5000:5000 time_server:latest

