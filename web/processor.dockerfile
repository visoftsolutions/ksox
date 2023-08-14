FROM node AS installer
WORKDIR /app
COPY . .
RUN npm ci

FROM installer AS builder
WORKDIR /app/apps/processor
RUN npm run build

FROM builder AS runtime
WORKDIR /app/apps/processor
ENTRYPOINT [ "npm", "run", "start", "--", "--port", "80" ]
