<script>
	import { SvelteUIProvider, globalCss } from '@svelteuidev/core';

	globalCss({
		body: {
			backgroundColor: '#b35c44',
			backgroundImage: 'url(/bg.texture.png), url(/bg.top.png);',
			backgroundRepeat: 'repeat, repeat-x'
		}
	})();
</script>

<SvelteUIProvider withNormalizeCSS>
	<slot />
</SvelteUIProvider>
