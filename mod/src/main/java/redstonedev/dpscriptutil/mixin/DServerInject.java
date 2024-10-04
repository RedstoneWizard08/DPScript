package redstonedev.dpscriptutil.mixin;

import net.minecraft.server.dedicated.DedicatedServer;

import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;
import redstonedev.dpscriptutil.Console;

@Mixin(DedicatedServer.class)
public abstract class DServerInject {
	@Inject(at = @At("HEAD"), method = "initServer()Z")
	private void captureServer(CallbackInfoReturnable<Boolean> info) {
		Console.server = (DedicatedServer) (Object) this;
	}
}
