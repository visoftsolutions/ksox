FROM node AS installer
WORKDIR /app
COPY . .
RUN npm ci

FROM installer AS runtime
WORKDIR /app
ENTRYPOINT [ "npm", "run", "node" ]
