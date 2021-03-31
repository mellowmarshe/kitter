FROM node:lts-alpine

RUN mkdir /frontend
RUN npm install -g http-server
WORKDIR /frontend
COPY package.json ./
RUN npm install
COPY . .

RUN npm run build

EXPOSE 8084
CMD [ "http-server", "dist", "-p", "8084" ]