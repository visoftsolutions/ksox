FROM node
WORKDIR /app
COPY . .
RUN npm ci --verbose
WORKDIR /app/@app/processor
RUN npm run build
ENV PORT=80
ENTRYPOINT [ "npm", "run", "start" ]
