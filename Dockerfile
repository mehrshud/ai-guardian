# Stage 1: Build Rust application
FROM rust:alpine as builder
RUN addgroup -g 1000 guardian && adduser -u 1000 -G guardian -s /bin/sh -D guardian
WORKDIR /app
COPY . .
RUN cargo build --release
RUN chown -R guardian:guardian /app

# Stage 2: Build TensorFlow model
FROM tensorflow/tensorflow:latest as model
WORKDIR /model
COPY model /model
RUN python -m pip install -r requirements.txt
RUN python train.py

# Stage 3: Assemble final image
FROM nginx:alpine
RUN addgroup -g 1000 guardian && adduser -u 1000 -G guardian -s /bin/sh -D guardian
WORKDIR /app
COPY --from=builder /app/target/release/ai-guardian /app
COPY --from=model /model /model
COPY nginx.conf /etc/nginx/nginx.conf
COPY index.html /usr/share/nginx/html
EXPOSE 80
HEALTHCHECK --interval=30s --timeout=5s --retries=3 CMD curl -f http://localhost:80/ || exit 1
USER guardian:guardian
CMD ["nginx", "-g", "daemon off;"]
