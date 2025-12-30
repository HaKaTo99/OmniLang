/** @type {import('next').NextConfig} */
const nextConfig = {
	webpack: (config) => {
		// Avoid bundling require-in-the-middle to silence critical dependency warning from opentelemetry stack
		config.externals = config.externals || [];
		config.externals.push({ "require-in-the-middle": "commonjs require-in-the-middle" });
		return config;
	},
};

export default nextConfig;
