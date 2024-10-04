package redstonedev.dpscriptutil.mixin;

import redstonedev.dpscriptutil.Console;
import redstonedev.dpscriptutil.DPScriptUtil;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Overwrite;

@Mixin(targets = "net.minecraft.server.dedicated.DedicatedServer$1")
public class DServerConsoleThreadInject {
    /**
     * @author Fourmisain
     * @reason Replaces the vanilla console handler logic
     */
    @Overwrite
    public void run() {
        DPScriptUtil.LOGGER.info("Starting {}", DPScriptUtil.MOD_ID);
        Console.run();
    }
}
