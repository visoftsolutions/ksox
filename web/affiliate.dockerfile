FROM node:20.5-alpine AS installer
WORKDIR /app
COPY . .
RUN npm ci

FROM installer AS builder
WORKDIR /app
ARG NODE_ENV
ENV NODE_ENV ${NODE_ENV}
RUN npm run build -- apps/affiliate

FROM builder AS runtime
WORKDIR /app
ENTRYPOINT [ "npm", "run", "start", "--", "apps/affiliate", "--", "--port", "80" ]