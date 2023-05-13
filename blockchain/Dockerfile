FROM node:lts
WORKDIR /app
COPY . .
RUN npm ci
ENTRYPOINT [ "npx", "hardhat", "node", "--hostname", "0.0.0.0" ]
