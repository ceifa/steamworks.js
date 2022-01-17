import typescript from '@rollup/plugin-typescript'
import url from '@rollup/plugin-url'
import externals from 'rollup-plugin-node-externals'

const production = !process.env.ROLLUP_WATCH

export default {
    input: './client/index.ts',
    output: {
        file: 'dist/index.js',
        format: 'cjs',
        name: 'steamworks.js',
        sourcemap: !production,
    },
    external: ['node'],
    plugins: [
        url({
            include: '**/*.exe',
            fileName: '[name][extname]',
        }),
        externals(),
        typescript({
            sourceMap: !production,
        }),
    ],
}
