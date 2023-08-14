FROM node AS installer
WORKDIR /app
COPY . .
RUN npm ci

FROM installer AS builder
WORKDIR /app
ARG NODE_ENV
ENV NODE_ENV ${NODE_ENV}
RUN npm run build -- apps/dashboard

FROM builder AS runtime
WORKDIR /app
ENTRYPOINT [ "npm", "run", "start", "--", "apps/dashboard", "--", "--port", "80" ]
