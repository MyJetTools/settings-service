class Utils {
    public static compileSettingsUrl(env: string, name: string): string {
        return "/settings/" + env + "/" + name;
    }
}