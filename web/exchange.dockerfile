FROM node AS installer
WORKDIR /app
COPY . .
RUN npm ci

FROM installer AS builder
WORKDIR /app/@app/exchange
RUN npm run build

FROM builder AS runtime
WORKDIR /app/@app/exchange
ENTRYPOINT [ "npm", "run", "start", "--", "--port", "80" ]
