import { defineConfig } from "vite";
import path from "path";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
    plugins: [solidPlugin()],
    server: {
        port: 3000
    },
    resolve: {
        alias: {
            "@state": path.resolve(__dirname, "./src/states"),
            "@types": path.resolve(__dirname, "./src/types"),
            "@icons": path.resolve(__dirname, "./src/components/Icons"),
            "@utils": path.resolve(__dirname, "./src/components/utils"),
            "@input": path.resolve(__dirname, "./src/components/Input")
        }
    }
});