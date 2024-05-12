FROM rust

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/app

COPY . .

# RUN diesel setup

RUN cargo install --path .

EXPOSE 8000

CMD ["passring-agency-server"]