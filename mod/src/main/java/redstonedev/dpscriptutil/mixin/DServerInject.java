package redstonedev.dpscriptutil.mixin;

import net.minecraft.server.dedicated.MinecraftDedicatedServer;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;
import redstonedev.dpscriptutil.Console;

@Mixin(MinecraftDedicatedServer.class)
public abstract class DServerInject
{
	@Inject(at = @At("HEAD"), method = "setupServer()Z")
	private void captureServer(CallbackInfoReturnable<Boolean> info)
	{
		Console.server = (MinecraftDedicatedServer) (Object) this;
	}
}
