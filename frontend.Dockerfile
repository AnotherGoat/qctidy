# syntax=docker/dockerfile:1

FROM node:20-alpine AS dependencies
WORKDIR /app
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci

FROM node:20-alpine AS build
WORKDIR /app
COPY --from=dependencies /app/node_modules ./node_modules
COPY frontend/ ./
RUN npm run build

FROM node:20-alpine
WORKDIR /app
ENV NODE_ENV=production
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci --omit=dev
COPY --from=build /app/build ./build
CMD ["npm", "run", "start"]
