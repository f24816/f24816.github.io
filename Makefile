site: index.html.template
	mkdir -p _site
	sed -e "s/__LAST_UPDATED__/$(DATE)/" index.html > _site/index.html
