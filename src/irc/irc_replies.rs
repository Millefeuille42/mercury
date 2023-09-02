use crate::irc::irc_errors::IRCError;
use crate::irc::irc_context::IRCContext;
use crate::irc::irc_message_handler::IRCMessageHandler;
use crate::irc::irc_message_parsed::IRCMessageParsed;

pub enum IRCReplies {
	RplWelcome,
	RplYourHost,
	RplCreated,
	RplMyInfo,
	RplBounce,
	RplUserHost,
	RplIsOn,
	RplAway,
	RplUnAway,
	RplNowAway,
	RplWhoIsUser,
	RplWhoIsServer,
	RplWhoIsOperator,
	RplWhoIsIdle,
	RplEndOfWhoIs,
	RplWhoIsChannels,
	RplWhoWasUser,
	RplEndOfWhoWas,
	RplListStart,
	RplList,
	RplListEnd,
	RplUniqOpIs,
	RplChannelModeIs,
	RplNoTopic,
	RplTopic,
	RplInviting,
	RplSummoning,
	RplInviteList,
	RplEndOfInviteList,
	RplExceptList,
	RplEndOfExceptList,
	RplVersion,
	RplWhoReply,
	RplEndOfWho,
	RplNameReply,
	RplEndOfNames,
	RplLinks,
	RplEndOfLinks,
	RplBanList,
	RplEndOfBanList,
	RplInfo,
	RplEndOfInfo,
	RplMotDStart,
	RplMotD,
	RplEndOfMotD,
	RplYouReOper,
	RplRehashing,
	RplYouReService,
	RplTime,
	RplUsersStart,
	RplUsers,
	RplEndOfUsers,
	RplNoUsers,
	RplTraceLink,
	RplTraceConnecting,
	RplTraceHandshake,
	RplTraceUnknown,
	RplTraceOperator,
	RplTraceUser,
	RplTraceServer,
	RplTraceService,
	RplTraceNewType,
	RplTraceClass,
	RplTraceReconnect,
	RplTraceLog,
	RplTraceEnd,
	RplStatsLinkInfo,
	RplStatsCommands,
	RplEndOfStats,
	RplStatsUptime,
	RplStatsOLine,
	RplUModeIs,
	RplServList,
	RplServListEnd,
	RplLUserClient,
	RplLUserOp,
	RplLUserUnknown,
	RplLUserChannels,
	RplLUserMe,
	RplAdminMe,
	RplAdminLoc1,
	RplAdminLoc2,
	RplAdminEmail,
	RplTryAgain,
	ErrNoSuchNick,
	ErrNoSuchServer,
	ErrNoSuchChannel,
	ErrCannotSendToChan,
	ErrTooManyChannels,
	ErrWasNoSuchNick,
	ErrTooManyTargets,
	ErrNoSuchService,
	ErrNoOrigin,
	ErrNoRecipient,
	ErrNoTextToSend,
	ErrNoToplevel,
	ErrWildTopLevel,
	ErrBadMask,
	ErrUnknownCommand,
	ErrNoMotD,
	ErrNoAdminInfo,
	ErrFileError,
	ErrNoNicknameGiven,
	ErrErroneusNickname,
	ErrNicknameInUse,
	ErrNickCollision,
	ErrUnavailResource,
	ErrUserNotInChannel,
	ErrNotOnChannel,
	ErrUserOnChannel,
	ErrNoLogin,
	ErrSummonDisabled,
	ErrUsersDisabled,
	ErrNotRegistered,
	ErrNeedMoreParams,
	ErrAlreadyRegistred,
	ErrNoPermForHost,
	ErrPasswdMismatch,
	ErrYouReBannedCreep,
	ErrYouWillBeBanned,
	ErrKeySet,
	ErrChannelIsFull,
	ErrUnknownMode,
	ErrInviteOnlyChan,
	ErrBannedFromChan,
	ErrBadChannelKey,
	ErrBadChanMask,
	ErrNoChanModes,
	ErrBanListFull,
	ErrNoPrivileges,
	ErrChanOPrivsNeeded,
	ErrCantKillServer,
	ErrRestricted,
	ErrUniqOpPrivsNeeded,
	ErrNoOperHost,
	ErrUModeUnknownFlag,
	ErrUsersDontMatch,
	RplHostHidden,
	RplLocalUsers,
	RplGlobalUsers,
	Unknown
}

impl IRCMessageHandler for IRCReplies {
	fn new(reply: &str) -> Result<IRCReplies, IRCError> {
		let reply = reply.to_uppercase();
		let reply = reply.as_str();
		let found = match reply {
			"001" => IRCReplies::RplWelcome, //RPL_WELCOME
			"002" => IRCReplies::RplYourHost, //RPL_YOURHOST
			"003" => IRCReplies::RplCreated, //RPL_CREATED
			"004" => IRCReplies::RplMyInfo, //RPL_MYINFO
			"005" => IRCReplies::RplBounce, //RPL_BOUNCE
			"302" => IRCReplies::RplUserHost, //RPL_USERHOST
			"303" => IRCReplies::RplIsOn, //RPL_ISON
			"301" => IRCReplies::RplAway, //RPL_AWAY
			"305" => IRCReplies::RplUnAway, //RPL_UNAWAY
			"306" => IRCReplies::RplNowAway, //RPL_NOWAWAY
			"311" => IRCReplies::RplWhoIsUser, //RPL_WHOISUSER
			"312" => IRCReplies::RplWhoIsServer, //RPL_WHOISSERVER
			"313" => IRCReplies::RplWhoIsOperator, //RPL_WHOISOPERATOR
			"317" => IRCReplies::RplWhoIsIdle, //RPL_WHOISIDLE
			"318" => IRCReplies::RplEndOfWhoIs, //RPL_ENDOFWHOIS
			"319" => IRCReplies::RplWhoIsChannels, //RPL_WHOISCHANNELS
			"314" => IRCReplies::RplWhoWasUser, //RPL_WHOWASUSER
			"369" => IRCReplies::RplEndOfWhoWas, //RPL_ENDOFWHOWAS
			"321" => IRCReplies::RplListStart, //RPL_LISTSTART
			"322" => IRCReplies::RplList, //RPL_LIST
			"323" => IRCReplies::RplListEnd, //RPL_LISTEND
			"325" => IRCReplies::RplUniqOpIs, //RPL_UNIQOPIS
			"324" => IRCReplies::RplChannelModeIs, //RPL_CHANNELMODEIS
			"331" => IRCReplies::RplNoTopic, //RPL_NOTOPIC
			"332" => IRCReplies::RplTopic, //RPL_TOPIC
			"341" => IRCReplies::RplInviting, //RPL_INVITING
			"342" => IRCReplies::RplSummoning, //RPL_SUMMONING
			"346" => IRCReplies::RplInviteList, //RPL_INVITELIST
			"347" => IRCReplies::RplEndOfInviteList, //RPL_ENDOFINVITELIST
			"348" => IRCReplies::RplExceptList, //RPL_EXCEPTLIST
			"349" => IRCReplies::RplEndOfExceptList, //RPL_ENDOFEXCEPTLIST
			"351" => IRCReplies::RplVersion, //RPL_VERSION
			"352" => IRCReplies::RplWhoReply, //RPL_WHOREPLY
			"315" => IRCReplies::RplEndOfWho, //RPL_ENDOFWHO
			"353" => IRCReplies::RplNameReply, //RPL_NAMREPLY
			"366" => IRCReplies::RplEndOfNames, //RPL_ENDOFNAMES
			"364" => IRCReplies::RplLinks, //RPL_LINKS
			"365" => IRCReplies::RplEndOfLinks, //RPL_ENDOFLINKS
			"367" => IRCReplies::RplBanList, //RPL_BANLIST
			"368" => IRCReplies::RplEndOfBanList, //RPL_ENDOFBANLIST
			"371" => IRCReplies::RplInfo, //RPL_INFO
			"374" => IRCReplies::RplEndOfInfo, //RPL_ENDOFINFO
			"375" => IRCReplies::RplMotDStart, //RPL_MOTDSTART
			"372" => IRCReplies::RplMotD, //RPL_MOTD
			"376" => IRCReplies::RplEndOfMotD, //RPL_ENDOFMOTD
			"381" => IRCReplies::RplYouReOper, //RPL_YOUREOPER
			"382" => IRCReplies::RplRehashing, //RPL_REHASHING
			"383" => IRCReplies::RplYouReService, //RPL_YOURESERVICE
			"391" => IRCReplies::RplTime, //RPL_TIME
			"392" => IRCReplies::RplUsersStart, //RPL_USERSSTART
			"393" => IRCReplies::RplUsers, //RPL_USERS
			"394" => IRCReplies::RplEndOfUsers, //RPL_ENDOFUSERS
			"395" => IRCReplies::RplNoUsers, //RPL_NOUSERS
			"200" => IRCReplies::RplTraceLink, //RPL_TRACELINK
			"201" => IRCReplies::RplTraceConnecting, //RPL_TRACECONNECTING
			"202" => IRCReplies::RplTraceHandshake, //RPL_TRACEHANDSHAKE
			"203" => IRCReplies::RplTraceUnknown, //RPL_TRACEUNKNOWN
			"204" => IRCReplies::RplTraceOperator, //RPL_TRACEOPERATOR
			"205" => IRCReplies::RplTraceUser, //RPL_TRACEUSER
			"206" => IRCReplies::RplTraceServer, //RPL_TRACESERVER
			"207" => IRCReplies::RplTraceService, //RPL_TRACESERVICE
			"208" => IRCReplies::RplTraceNewType, //RPL_TRACENEWTYPE
			"209" => IRCReplies::RplTraceClass, //RPL_TRACECLASS
			"210" => IRCReplies::RplTraceReconnect, //RPL_TRACERECONNECT
			"261" => IRCReplies::RplTraceLog, //RPL_TRACELOG
			"262" => IRCReplies::RplTraceEnd, //RPL_TRACEEND
			"211" => IRCReplies::RplStatsLinkInfo, //RPL_STATSLINKINFO
			"212" => IRCReplies::RplStatsCommands, //RPL_STATSCOMMANDS
			"219" => IRCReplies::RplEndOfStats, //RPL_ENDOFSTATS
			"242" => IRCReplies::RplStatsUptime, //RPL_STATSUPTIME
			"243" => IRCReplies::RplStatsOLine, //RPL_STATSOLINE
			"221" => IRCReplies::RplUModeIs, //RPL_UMODEIS
			"234" => IRCReplies::RplServList, //RPL_SERVLIST
			"235" => IRCReplies::RplServListEnd, //RPL_SERVLISTEND
			"251" => IRCReplies::RplLUserClient, //RPL_LUSERCLIENT
			"252" => IRCReplies::RplLUserOp, //RPL_LUSEROP
			"253" => IRCReplies::RplLUserUnknown, //RPL_LUSERUNKNOWN
			"254" => IRCReplies::RplLUserChannels, //RPL_LUSERCHANNELS
			"255" => IRCReplies::RplLUserMe, //RPL_LUSERME
			"256" => IRCReplies::RplAdminMe, //RPL_ADMINME
			"257" => IRCReplies::RplAdminLoc1, //RPL_ADMINLOC1
			"258" => IRCReplies::RplAdminLoc2, //RPL_ADMINLOC2
			"259" => IRCReplies::RplAdminEmail, //RPL_ADMINEMAIL
			"263" => IRCReplies::RplTryAgain, //RPL_TRYAGAIN
			"401" => IRCReplies::ErrNoSuchNick, //ERR_NOSUCHNICK
			"402" => IRCReplies::ErrNoSuchServer, //ERR_NOSUCHSERVER
			"403" => IRCReplies::ErrNoSuchChannel, //ERR_NOSUCHCHANNEL
			"404" => IRCReplies::ErrCannotSendToChan, //ERR_CANNOTSENDTOCHAN
			"405" => IRCReplies::ErrTooManyChannels, //ERR_TOOMANYCHANNELS
			"406" => IRCReplies::ErrWasNoSuchNick, //ERR_WASNOSUCHNICK
			"407" => IRCReplies::ErrTooManyTargets, //ERR_TOOMANYTARGETS
			"408" => IRCReplies::ErrNoSuchService, //ERR_NOSUCHSERVICE
			"409" => IRCReplies::ErrNoOrigin, //ERR_NOORIGIN
			"411" => IRCReplies::ErrNoRecipient, //ERR_NORECIPIENT
			"412" => IRCReplies::ErrNoTextToSend, //ERR_NOTEXTTOSEND
			"413" => IRCReplies::ErrNoToplevel, //ERR_NOTOPLEVEL
			"414" => IRCReplies::ErrWildTopLevel, //ERR_WILDTOPLEVEL
			"415" => IRCReplies::ErrBadMask, //ERR_BADMASK
			"421" => IRCReplies::ErrUnknownCommand, //ERR_UNKNOWNCOMMAND
			"422" => IRCReplies::ErrNoMotD, //ERR_NOMOTD
			"423" => IRCReplies::ErrNoAdminInfo, //ERR_NOADMININFO
			"424" => IRCReplies::ErrFileError, //ERR_FILEERROR
			"431" => IRCReplies::ErrNoNicknameGiven, //ERR_NONICKNAMEGIVEN
			"432" => IRCReplies::ErrErroneusNickname, //ERR_ERRONEUSNICKNAME
			"433" => IRCReplies::ErrNicknameInUse, //ERR_NICKNAMEINUSE
			"436" => IRCReplies::ErrNickCollision, //ERR_NICKCOLLISION
			"437" => IRCReplies::ErrUnavailResource, //ERR_UNAVAILRESOURCE
			"441" => IRCReplies::ErrUserNotInChannel, //ERR_USERNOTINCHANNEL
			"442" => IRCReplies::ErrNotOnChannel, //ERR_NOTONCHANNEL
			"443" => IRCReplies::ErrUserOnChannel, //ERR_USERONCHANNEL
			"444" => IRCReplies::ErrNoLogin, //ERR_NOLOGIN
			"445" => IRCReplies::ErrSummonDisabled, //ERR_SUMMONDISABLED
			"446" => IRCReplies::ErrUsersDisabled, //ERR_USERSDISABLED
			"451" => IRCReplies::ErrNotRegistered, //ERR_NOTREGISTERED
			"461" => IRCReplies::ErrNeedMoreParams, //ERR_NEEDMOREPARAMS
			"462" => IRCReplies::ErrAlreadyRegistred, //ERR_ALREADYREGISTRED
			"463" => IRCReplies::ErrNoPermForHost, //ERR_NOPERMFORHOST
			"464" => IRCReplies::ErrPasswdMismatch, //ERR_PASSWDMISMATCH
			"465" => IRCReplies::ErrYouReBannedCreep, //ERR_YOUREBANNEDCREEP
			"466" => IRCReplies::ErrYouWillBeBanned, //ERR_YOUWILLBEBANNED
			"467" => IRCReplies::ErrKeySet, //ERR_KEYSET
			"471" => IRCReplies::ErrChannelIsFull, //ERR_CHANNELISFULL
			"472" => IRCReplies::ErrUnknownMode, //ERR_UNKNOWNMODE
			"473" => IRCReplies::ErrInviteOnlyChan, //ERR_INVITEONLYCHAN
			"474" => IRCReplies::ErrBannedFromChan, //ERR_BANNEDFROMCHAN
			"475" => IRCReplies::ErrBadChannelKey, //ERR_BADCHANNELKEY
			"476" => IRCReplies::ErrBadChanMask, //ERR_BADCHANMASK
			"477" => IRCReplies::ErrNoChanModes, //ERR_NOCHANMODES
			"478" => IRCReplies::ErrBanListFull, //ERR_BANLISTFULL
			"481" => IRCReplies::ErrNoPrivileges, //ERR_NOPRIVILEGES
			"482" => IRCReplies::ErrChanOPrivsNeeded, //ERR_CHANOPRIVSNEEDED
			"483" => IRCReplies::ErrCantKillServer, //ERR_CANTKILLSERVER
			"484" => IRCReplies::ErrRestricted, //ERR_RESTRICTED
			"485" => IRCReplies::ErrUniqOpPrivsNeeded, //ERR_UNIQOPPRIVSNEEDED
			"491" => IRCReplies::ErrNoOperHost, //ERR_NOOPERHOST
			"501" => IRCReplies::ErrUModeUnknownFlag, //ERR_UMODEUNKNOWNFLAG
			"502" => IRCReplies::ErrUsersDontMatch, //ERR_USERSDONTMATCH
			"396" => IRCReplies::RplHostHidden, //RPL_HOSTHIDDEN
			"265" => IRCReplies::RplLocalUsers, //RPL_HOSTHIDDEN
			"266" => IRCReplies::RplGlobalUsers, //RPL_HOSTHIDDEN
			_ => IRCReplies::Unknown
		};

		match found {
			IRCReplies::Unknown => Err(IRCError::ReplyNotFound(reply.to_string())),
			_ => Ok(found)
		}
	}

	fn format(&self, message: IRCMessageParsed) -> String {
		match self {
			IRCReplies::RplWelcome => format_print_nick_and_data(message),
			IRCReplies::RplYourHost => format_print_nick_and_data(message),
			IRCReplies::RplCreated => format_print_nick_and_data(message),
			IRCReplies::RplMyInfo  => format_print_nick_and_data(message),
			IRCReplies::RplLUserClient  => format_print_nick_and_data(message),
			IRCReplies::RplLUserOp  => format_print_nick_and_data(message),
			IRCReplies::RplLUserUnknown  => format_print_nick_and_data(message),
			IRCReplies::RplLUserChannels  => format_print_nick_and_data(message),
			IRCReplies::RplLUserMe  => format_print_nick_and_data(message),
			IRCReplies::RplLocalUsers  => format_print_nick_and_data(message),
			IRCReplies::RplGlobalUsers  => format_print_nick_and_data(message),
			IRCReplies::RplHostHidden => format_print_nick_and_data(message),
			IRCReplies::RplMotD => format_print_data(message),
			IRCReplies::RplMotDStart => format_motd_start(),
			IRCReplies::RplEndOfMotD => format_motd_end(),
			_ => message.as_raw()
		}
	}

	fn craft(&self, command: &str, _: &str, _: IRCContext) -> Result<IRCMessageParsed, IRCError> {
		match self {
			_ => Err(IRCError::ReplyNotFound(command.to_string()))
		}
	}
}

fn format_print_nick_and_data(message: IRCMessageParsed) -> String {
	let nick = message.parse_prefix().nick;
	format!("{}: {}", nick, message.data)
}

fn format_print_data(message: IRCMessageParsed) -> String {
	message.data.to_string()
}

fn format_motd_start() -> String {
	"--- Start of MotD ---".to_string()
}
fn format_motd_end() -> String {
	"--- End of MotD ---".to_string()
}
