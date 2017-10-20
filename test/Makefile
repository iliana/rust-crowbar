include Makefile.local
include Makefile.docker

list:
	@$(MAKE) -pRrq -f $(MAKEFILE_LIST) : 2>/dev/null | awk -v RS= -F: '/^# File/,/^# Finished Make/ { if ($$1 !~ "^[#.]") { print $$1 }}' | sort
