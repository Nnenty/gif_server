build:
    docker build -t gif_server .

run: build
    docker run --rm -p 8888:5000 --name gif_server gif_server
