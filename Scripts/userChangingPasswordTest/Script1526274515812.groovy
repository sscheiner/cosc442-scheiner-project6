import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.phptravels.net/login')

WebUI.setText(findTestObject('Page_Login (13)/input_username'), 'user@phptravels.com')

WebUI.setText(findTestObject('Page_Login (12)/input_password'), 'demouser')

WebUI.sendKeys(findTestObject('Page_Login (12)/input_password'), Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Page_My Account (9)/a_DVhbCERv'))

WebUI.click(findTestObject('Page_My Account (9)/a_Account'))

WebUI.click(findTestObject('Page_My Account (9)/a_My Profile'))

WebUI.setText(findTestObject('Page_My Account (9)/input_password'), 'newpass')

WebUI.setText(findTestObject('Page_My Account (9)/input_confirmpassword'), 'newpass')

WebUI.click(findTestObject('Page_My Account (9)/button_Submit'))

WebUI.click(findTestObject('Page_My Account (9)/a_DVhbCERv'))

WebUI.click(findTestObject('Page_My Account (9)/a_Logout'))

WebUI.setText(findTestObject('Page_Login (13)/input_username'), 'user@phptravels.com')

WebUI.setText(findTestObject('Page_Login (12)/input_password'), 'newpass')

WebUI.click(findTestObject('Page_Login (12)/button_Login'))

WebUI.verifyElementPresent(findTestObject('Page_My Account (9)/div_Hi DVhbCERv IlqEZZxz'), 0)

WebUI.closeBrowser()

