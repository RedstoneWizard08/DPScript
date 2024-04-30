package redstonedev.dpscriptutil;

import com.mojang.brigadier.CommandDispatcher;
import com.mojang.brigadier.context.CommandContext;
import com.mojang.brigadier.suggestion.SuggestionProvider;

import net.minecraft.command.CommandSource;
import net.minecraft.command.DataCommandStorage;
import net.minecraft.command.argument.IdentifierArgumentType;
import net.minecraft.command.argument.NbtElementArgumentType;
import net.minecraft.command.argument.NbtPathArgumentType;
import net.minecraft.nbt.NbtDouble;
import net.minecraft.nbt.NbtFloat;
import net.minecraft.nbt.NbtInt;
import net.minecraft.server.command.ServerCommandSource;

import org.quiltmc.qsl.command.api.EnumArgumentType;

import static net.minecraft.server.command.CommandManager.literal;
import static net.minecraft.server.command.CommandManager.argument;

public class DataCommandOps {
    public static final SuggestionProvider<ServerCommandSource> SUGGESTION_PROVIDER = (context,
            builder) -> CommandSource.suggestIdentifiers(
                    of(context).getIds(), builder);

    static DataCommandStorage of(CommandContext<ServerCommandSource> context) {
        return context.getSource().getServer().getDataCommandStorage();
    }

    public static void register(CommandDispatcher<ServerCommandSource> dispatcher) {
        var builder = literal("data")
                .requires((serverCommandSource) -> {
                    return serverCommandSource.hasPermission(2);
                });

        dispatcher.register(builder.then(literal("operation").then(
                literal("storage").then(argument("storage", IdentifierArgumentType.identifier())
                        .suggests(SUGGESTION_PROVIDER)
                        .then(argument("path", NbtPathArgumentType.nbtPath()).then(argument("operation",
                                new EnumArgumentType("add", "subtract", "sub", "multiply", "mul", "divide", "div",
                                        "power", "pow", "and", "or", "xor"))
                                .then(argument("value", NbtElementArgumentType.nbtElement()).executes(ctx -> {
                                    var storageId = IdentifierArgumentType.getIdentifier(ctx, "storage");
                                    var path = NbtPathArgumentType.getNbtPath(ctx, "path");
                                    var operation = EnumArgumentType.getEnum(ctx, "operation");
                                    var value = NbtElementArgumentType.getNbtElement(ctx, "value");
                                    var server = ctx.getSource().getServer();
                                    var storage = server.getDataCommandStorage().get(storageId);
                                    var objects = path.get(storage);
                                    var object = objects.get(objects.size() - 1);
                                    float originalValue;
                                    float added;

                                    if (object instanceof NbtFloat) {
                                        originalValue = ((NbtFloat) object).floatValue();
                                    } else if (object instanceof NbtDouble) {
                                        originalValue = (float) ((NbtDouble) object).doubleValue();
                                    } else if (object instanceof NbtInt) {
                                        originalValue = (float) ((NbtInt) object).intValue();
                                    } else {
                                        return -1;
                                    }

                                    if (value instanceof NbtFloat) {
                                        added = ((NbtFloat) value).floatValue();
                                    } else if (value instanceof NbtDouble) {
                                        added = (float) ((NbtDouble) value).doubleValue();
                                    } else if (value instanceof NbtInt) {
                                        added = (float) ((NbtInt) value).intValue();
                                    } else {
                                        return -1;
                                    }

                                    switch (operation) {
                                        case "add" -> { originalValue += added; }
                                        case "subtract", "sub" -> { originalValue -= added; }
                                        case "multiply", "mul" -> { originalValue *= added; }
                                        case "divide", "div" -> { originalValue /= added; }
                                        case "power", "pow" -> { originalValue = (float) Math.pow((double) originalValue, (double) added); }
                                        case "and" -> { originalValue = (int) originalValue & (int) added; }
                                        case "or" -> { originalValue = (int) originalValue | (int) added; }
                                        case "xor" -> { originalValue = (int) originalValue ^ (int) added; }
                                    }

                                    path.set(storage, NbtFloat.of(originalValue));
                                    server.getDataCommandStorage().set(storageId, storage);

                                    return (int) originalValue;
                                }))))))));
    }
}
