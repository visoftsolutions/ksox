FROM node AS installer
WORKDIR /app
COPY . .
RUN npm ci

FROM installer AS builder
WORKDIR /app/@app/processor
RUN npm run build

FROM builder AS runtime
WORKDIR /app/@app/processor
ENTRYPOINT [ "npm", "run", "start", "--", "--port", "80" ]
