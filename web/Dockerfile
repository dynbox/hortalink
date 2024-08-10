FROM node:lts-alpine

RUN npm install -g pnpm

WORKDIR /usr/src/app

COPY . .
COPY ../pnpm-workspace.yaml ./usr/src/app/

COPY package*.json ./
RUN pnpm install

CMD ["pnpm", "recursive", "run", "dev"]