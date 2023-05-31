FROM node
WORKDIR /app
COPY . .
RUN npm ci
RUN npm run build
ENV PORT=80
ENTRYPOINT [ "npm", "run", "start" ]
