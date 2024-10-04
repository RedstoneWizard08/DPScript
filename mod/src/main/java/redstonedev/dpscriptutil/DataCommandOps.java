package redstonedev.dpscriptutil;

import com.mojang.brigadier.CommandDispatcher;
import com.mojang.brigadier.context.CommandContext;
import com.mojang.brigadier.suggestion.SuggestionProvider;

import net.minecraft.commands.CommandSourceStack;
import net.minecraft.commands.SharedSuggestionProvider;
import net.minecraft.commands.arguments.NbtPathArgument;
import net.minecraft.commands.arguments.ResourceLocationArgument;
import net.minecraft.nbt.DoubleTag;
import net.minecraft.nbt.FloatTag;
import net.minecraft.nbt.IntTag;
import net.minecraft.util.Mth;
import net.minecraft.world.level.storage.CommandStorage;

import org.quiltmc.qsl.command.api.EnumArgumentType;
import java.util.function.Function;

import static net.minecraft.commands.Commands.literal;
import static net.minecraft.commands.Commands.argument;

public class DataCommandOps {
    public static final SuggestionProvider<CommandSourceStack> SUGGESTION_PROVIDER = (context,
            builder) -> SharedSuggestionProvider.suggestResource(
                    of(context).keys(), builder);

    private static CommandStorage of(CommandContext<CommandSourceStack> context) {
        return context.getSource().getServer().getCommandStorage();
    }

    public static void register(CommandDispatcher<CommandSourceStack> dispatcher) {
        var builder = literal("data")
                .requires((serverCommandSource) -> {
                    return serverCommandSource.hasPermission(2);
                });

        dispatcher.register(builder.then(literal("ops").then(
                literal("storage").then(argument("storage", ResourceLocationArgument.id())
                        .suggests(SUGGESTION_PROVIDER)
                        .then(argument("path", NbtPathArgument.nbtPath()).then(argument("operation",
                                new EnumArgumentType("add", "subtract", "sub", "multiply", "mul", "divide", "div",
                                        "power", "pow", "and", "or", "xor"))
                                .then(argument("storage2", ResourceLocationArgument.id())
                                        .then(argument("path2", NbtPathArgument.nbtPath()).executes(ctx -> {
                                            var storageId = ResourceLocationArgument.getId(ctx, "storage");
                                            var path = NbtPathArgument.getPath(ctx, "path");
                                            var rhsStorageId = ResourceLocationArgument.getId(ctx, "storage2");
                                            var rhsPath = NbtPathArgument.getPath(ctx, "path2");
                                            var operation = EnumArgumentType.getEnum(ctx, "operation");

                                            var server = ctx.getSource().getServer();
                                            var storage = server.getCommandStorage().get(storageId);
                                            var objects = path.get(storage);
                                            var object = objects.get(objects.size() - 1);

                                            var storage2 = server.getCommandStorage().get(rhsStorageId);
                                            var objects2 = rhsPath.get(storage2);
                                            var value = objects2.get(objects2.size() - 1);

                                            float originalValue;
                                            float added;

                                            if (object instanceof FloatTag) {
                                                originalValue = ((FloatTag) object).getAsFloat();
                                            } else if (object instanceof DoubleTag) {
                                                originalValue = (float) ((DoubleTag) object).getAsDouble();
                                            } else if (object instanceof IntTag) {
                                                originalValue = (float) ((IntTag) object).getAsInt();
                                            } else {
                                                return -1;
                                            }

                                            if (value instanceof FloatTag) {
                                                added = ((FloatTag) value).getAsFloat();
                                            } else if (value instanceof DoubleTag) {
                                                added = (float) ((DoubleTag) value).getAsDouble();
                                            } else if (value instanceof IntTag) {
                                                added = (float) ((IntTag) value).getAsInt();
                                            } else {
                                                return -1;
                                            }

                                            switch (operation) {
                                                case "add" -> {
                                                    originalValue += added;
                                                }
                                                case "subtract", "sub" -> {
                                                    originalValue -= added;
                                                }
                                                case "multiply", "mul" -> {
                                                    originalValue *= added;
                                                }
                                                case "divide", "div" -> {
                                                    originalValue /= added;
                                                }
                                                case "power", "pow" -> {
                                                    originalValue = (float) Math.pow((double) originalValue,
                                                            (double) added);
                                                }
                                                case "and" -> {
                                                    originalValue = (int) originalValue & (int) added;
                                                }
                                                case "or" -> {
                                                    originalValue = (int) originalValue | (int) added;
                                                }
                                                case "xor" -> {
                                                    originalValue = (int) originalValue ^ (int) added;
                                                }
                                            }

                                            path.set(storage, FloatTag.valueOf(originalValue));
                                            server.getCommandStorage().set(storageId, storage);

                                            return (int) originalValue;
                                        })))))))));

        dispatcher.register(builder.then(literal("advops").then(
                literal("storage").then(argument("storage", ResourceLocationArgument.id())
                        .suggests(SUGGESTION_PROVIDER)
                        .then(argument("path", NbtPathArgument.nbtPath()).then(argument("operation",
                                new EnumArgumentType("sqrt", "sin", "cos", "tan", "asin", "acos", "atan"))
                                .executes(ctx -> {
                                    var storageId = ResourceLocationArgument.getId(ctx, "storage");
                                    var path = NbtPathArgument.getPath(ctx, "path");
                                    var operation = EnumArgumentType.getEnum(ctx, "operation");

                                    var server = ctx.getSource().getServer();
                                    var storage = server.getCommandStorage().get(storageId);
                                    var objects = path.get(storage);
                                    var object = objects.get(objects.size() - 1);

                                    float originalValue;

                                    if (object instanceof FloatTag) {
                                        originalValue = ((FloatTag) object).getAsFloat();
                                    } else if (object instanceof DoubleTag) {
                                        originalValue = (float) ((DoubleTag) object).getAsDouble();
                                    } else if (object instanceof IntTag) {
                                        originalValue = (float) ((IntTag) object).getAsInt();
                                    } else {
                                        return -1;
                                    }

                                    Function<Float, Float> func = (f) -> f;

                                    switch (operation) {
                                        case "sqrt" -> {
                                            func = Mth::sqrt;
                                        }
                                        case "sin" -> {
                                            func = Mth::sin;
                                        }
                                        case "cos" -> {
                                            func = Mth::cos;
                                        }
                                        case "tan" -> {
                                            func = (f) -> (float) Math.tan((double) f);
                                        }
                                        case "asin" -> {
                                            func = (f) -> (float) Math.asin((double) f);
                                        }
                                        case "acos" -> {
                                            func = (f) -> (float) Math.acos((double) f);
                                        }
                                        case "atan" -> {
                                            func = (f) -> (float) Math.atan((double) f);
                                        }
                                    }

                                    originalValue = func.apply(originalValue);

                                    path.set(storage, FloatTag.valueOf(originalValue));
                                    server.getCommandStorage().set(storageId, storage);

                                    return (int) originalValue;
                                })))))));
    }
}
