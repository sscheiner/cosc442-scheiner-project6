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

WebUI.setText(findTestObject('Page_Login (10)/input_username'), 'user@phptravels.com')

WebUI.setText(findTestObject('Page_Login (10)/input_password'), 'demouser')

WebUI.click(findTestObject('Page_Login (10)/button_Login'))

WebUI.click(findTestObject('Page_My Account (7)/a_Flights'))

WebUI.setText(findTestObject('Page_Flights (4)/input_airports-inline-orig-53f'), 'bwi')

WebUI.sendKeys(findTestObject('Page_Flights (4)/input_airports-inline-orig-53f'), Keys.chord(Keys.ENTER))

WebUI.setText(findTestObject('Page_Flights (4)/input_airports-inline-dest-53f'), 'mke')

WebUI.sendKeys(findTestObject('Page_Flights (4)/input_airports-inline-dest-53f'), Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Page_Flights (4)/button_31'))

WebUI.click(findTestObject('Page_Flights (4)/span_Continue with 1 traveller'))

WebUI.click(findTestObject('Page_Flights (4)/span_Search'))

WebUI.verifyElementPresent(findTestObject('Page_Flights (4)/span_Filter Results'), 0)

WebUI.closeBrowser()

