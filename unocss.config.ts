import { presetWind } from '@unocss/preset-wind3'
import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetTypography,
  transformerDirectives,
  transformerVariantGroup,
} from 'unocss'
import { presetAnimations } from 'unocss-preset-animations'
import { presetScrollbar } from 'unocss-preset-scrollbar'
import { presetShadcn } from 'unocss-preset-shadcn'

export default defineConfig({
  presets: [
    presetWind(),
    presetAttributify(),
    presetIcons({
      scale: 1.2,
      extraProperties: {
        'display': 'inline-block',
        'vertical-align': 'middle',
      },
    }),
    presetTypography(),
    presetScrollbar(),
    presetAnimations(),
    presetShadcn({
      color: {
        base: 'red',
        light: { primary: '0 78% 62%', ring: '0 78% 62%' },
      },
    }),
  ],
  transformers: [transformerDirectives(), transformerVariantGroup()],
  content: {
    pipeline: {
      include: [
        /\.(vue|svelte|[jt]sx|mdx?|astro|elm|php|phtml|html)($|\?)/,
        '(components|src)/**/*.{js,ts}',
      ],
    },
  },
  theme: {
    colors: {
      nb: {
        50: '#fef2f2',
        100: '#fde3e3',
        200: '#fccccc',
        300: '#f9a8a8',
        400: '#f37676',
        500: '#ea5252',
        600: '#d62c2c',
        700: '#b42121',
        800: '#951f1f',
        900: '#7c2020',
        950: '#430c0c',
        primary: '#ea5252',
      },
    },
  },
})
