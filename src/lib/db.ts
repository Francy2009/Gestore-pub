import { PrismaClient } from '@prisma/client';
import path from 'node:path';

// Dynamically resolve the absolute path to dev.db in the project root's prisma folder
const dbPath = path.resolve(process.cwd(), 'prisma/dev.db');

const globalForPrisma = globalThis as unknown as {
  prisma: PrismaClient | undefined;
};

export const prisma =
  globalForPrisma.prisma ??
  new PrismaClient({
    datasources: {
      db: {
        url: `file:${dbPath}`,
      },
    },
    log: process.env.NODE_ENV === 'development' ? ['query'] : [],
  });

if (process.env.NODE_ENV !== 'production') {
  globalForPrisma.prisma = prisma;
}
