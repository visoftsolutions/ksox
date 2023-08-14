FROM node AS installer
WORKDIR /app
COPY . .
RUN npm ci

FROM installer AS builder
WORKDIR /app
RUN npm run build -- apps/exchange-landing

FROM builder AS runtime
WORKDIR /app
ENTRYPOINT [ "npm", "run", "start", "--", "apps/exchange-landing", "--", "--port", "80" ]
