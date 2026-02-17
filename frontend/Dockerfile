FROM node:20-alpine AS builder
WORKDIR /app

COPY package.json package-lock.json* ./
RUN npm install

COPY . .
RUN npm run build

FROM node:20-alpine AS runtime
WORKDIR /app

ENV NODE_ENV=production
RUN addgroup -g 1001 -S nodejs && adduser -S nuxt -u 1001

COPY --from=builder --chown=nuxt:nodejs /app/.output /app/.output
COPY --from=builder --chown=nuxt:nodejs /app/package.json /app/package.json

USER nuxt
EXPOSE 3000
ENV HOST=0.0.0.0
ENV PORT=3000
CMD ["node", ".output/server/index.mjs"]
