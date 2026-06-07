import { defineConfig } from 'vite'
import { devtools } from '@tanstack/devtools-vite'

import { tanstackStart } from '@tanstack/react-start/plugin/vite'

import viteReact from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import basicSsl from '@vitejs/plugin-basic-ssl'

const config = defineConfig({
  resolve: { tsconfigPaths: true },
  plugins: [basicSsl(), devtools(), tailwindcss(), tanstackStart(), viteReact()],
})

export default config
