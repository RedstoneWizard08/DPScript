package redstonedev.dpscriptutil;

import org.apache.logging.log4j.core.LogEvent;
import org.apache.logging.log4j.core.appender.AbstractAppender;
import org.apache.logging.log4j.core.appender.rewrite.RewritePolicy;
import org.apache.logging.log4j.core.layout.PatternLayout;
import org.jline.reader.LineReader;

import static redstonedev.dpscriptutil.Console.applyMinecraftStyle;

public class JLineAppender extends AbstractAppender {
	protected final LineReader lr;
	protected RewritePolicy policy;

	@SuppressWarnings("deprecation") // allows running on 1.16 and 1.17+
	public JLineAppender(LineReader lr) {
		super("JLine", null, PatternLayout.newBuilder().withPattern(DPScriptUtil.CONFIG.logPattern).build(), false);
		this.lr = lr;
	}

	public void setRewritePolicy(RewritePolicy policy) {
		this.policy = policy;
	}

	@Override
	public void append(LogEvent event) {
		if (policy != null)
			event = policy.rewrite(event);

		if (lr.isReading())
			lr.callWidget(LineReader.CLEAR);

		String s = getLayout().toSerializable(event).toString();

		if (DPScriptUtil.CONFIG.applyMinecraftStyle)
			s = applyMinecraftStyle(s);

		lr.getTerminal().writer().print(s);

		if (lr.isReading()) {
			lr.callWidget(LineReader.REDRAW_LINE);
			lr.callWidget(LineReader.REDISPLAY);
		}

		lr.getTerminal().writer().flush();
	}
}
