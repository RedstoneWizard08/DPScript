package redstonedev.dpscriptutil;

import org.quiltmc.loader.api.ModContainer;
import org.quiltmc.qsl.base.api.entrypoint.ModInitializer;
import org.quiltmc.qsl.command.api.CommandRegistrationCallback;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class DPScriptUtil implements ModInitializer {
	public static final String MOD_ID = "dpscriptutil";
    public static final Logger LOGGER = LoggerFactory.getLogger(MOD_ID);
	public static final ModConfig CONFIG;

	static {
		CONFIG = new ModConfig();

		try {
			CONFIG.read();
		} catch (Exception e) {
			LOGGER.error("couldn't fully read config file! {}", e.getMessage());
		}

		try {
			CONFIG.write();
		} catch (Exception e) {
			LOGGER.error("couldn't write config file! {}", e.getMessage());
		}
	}


    @Override
    public void onInitialize(ModContainer mod) {
        CommandRegistrationCallback.EVENT.register((disp, ctx, env) -> {
            DataCommandOps.register(disp);
        });
    }
}
