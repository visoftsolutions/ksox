FROM node
WORKDIR /app
COPY . .
RUN npm ci --verbose
WORKDIR /app/@app/dashboard
RUN npm run build
ENV PORT=80
ENTRYPOINT [ "npm", "run", "start" ]
